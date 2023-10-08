from multiprocessing import Process, Event
import random
import time
import uuid
import zmq


class Job(object):
    def __init__(self, work):
        self.id = uuid.uuid4().hex
        self.work = work

class Controller:
    def __init__(self, control_port=CONTROL_PORT):
        self.context = zmq.Context()
        self.socket = self.context.socket(zmq.ROUTER)
        self.socket.bind('tcp://*:{0}'.format(control_port))
        # We'll keep our workers here, this will be keyed on the worker id,
        # and the value will be a dict of Job instances keyed on job id.
        self.workers = {}
        # We won't assign more than 50 jobs to a worker at a time; this ensures
        # reasonable memory usage, and less shuffling when a worker dies.
        self.max_jobs_per_worker = 50

    def _get_next_worker_id(self):
        """Return the id of the next worker available to process work. Note
        that this will return None if no clients are available.
        """
        # It isn't strictly necessary since we're limiting the amount of work
        # we assign, but just to demonstrate that we could have any
        # algorithm here that we wanted we'll find the worker with the least
        # work and try that.
        worker_id, work = sorted(workers.items(), key=lambda x: len(x[1]))[0]
        if len(work) < self.max_jobs_per_worker:
            return worker_id
        # No worker is available. Our caller will have to handle this.
        return None
    
    def work_iterator(self):
        # iter() makes our xrange object into an iterator so we can use
        # next() on it.
        iterator = iter(xrange(0, 10000))
        while True:
            # Return requeued work first. We could simplify this method by
            # returning all new work then all requeued work, but this way we
            # know _work_to_requeue won't grow too large in the case of
            # many disconnects.
            if self._work_to_requeue:
                yield self._work_to_requeue.pop()
            else:
                yield Job({'number': next(iterator)})

    def _handle_worker_message(self, worker_id, message):
        """Handle a message from the worker identified by worker_id.

        {'message': 'connect'}
        {'message': 'disconnect'}
        {'message': 'job_done', 'job_id': 'xxx', 'result': 'yyy'}
        """
        if message['message'] == 'connect':
            assert worker_id not in self.workers
            self.workers[worker_id] = {}
        elif message['message'] == 'disconnect':
            remaining_work = self.workers.pop(worker_id)
            # Remove the worker so no more work gets added, and put any
            # remaining work into _work_to_requeue
            self._work_to_requeue.extend(remaining_work.values())
        elif message['message'] == 'work_done':
            result = message['result']
            job = self.workers[worker_id].pop([message['job_id']])
            # _process_results() is just a trivial logging function so I've
            # omitted it from here, but you can find it in the final source
            # code.
            self._process_results(worker_id, job, result)

    def _handle_worker_message(self, worker_id, message):
        """Handle a message from the worker identified by worker_id.

        {'message': 'connect'}
        {'message': 'disconnect'}
        {'message': 'job_done', 'job_id': 'xxx', 'result': 'yyy'}
        """
        if message['message'] == 'connect':
            assert worker_id not in self.workers
            self.workers[worker_id] = {}
        elif message['message'] == 'disconnect':
            remaining_work = self.workers.pop(worker_id)
            # Remove the worker so no more work gets added, and put any
            # remaining work into _work_to_requeue
            self._work_to_requeue.extend(remaining_work.values())
        elif message['message'] == 'work_done':
            result = message['result']
            job = self.workers[worker_id].pop([message['job_id']])
            # _process_results() is just a trivial logging function so I've
            # omitted it from here, but you can find it in the final source
            # code.
            self._process_results(worker_id, job, result)

class Worker:
    """Accept work in the form of {'number': xxx}, square the number and
    send it back to the controller in the form
    {'result': xxx, 'worker_id': yyy}. Our "work" in this case is just
    squaring the contents of 'number'.
    """
    def __init__(self, stop_event):
        self.stop_event = stop_event
        self.context = zmq.Context()
        self.socket = self.context.socket(zmq.DEALER)
        self.socket.identity = uuid.uudi4().hex[:4]
        self.socket.connect('tcp://127.0.0.1:5755')
        # self.socket_result = self.context.socket(zmq.PUSH)
        # self.socket_result.connect('tcp://127.0.0.1:5756')
        # We'll send this id back with our results to make it easier
        # to verify that work is getting distributed among multiple
        # workers.
        # self.my_id = uuid.uuid4().hex[:4]

    def run(self):
        try:
            # Send a connect message
            self.socket.send_json({'message': 'connect'})
            # Poll the socket for incoming messages. This will wait up to
            # 0.1 seconds before returning False. The other way to do this
            # is is to use zmq.NOBLOCK when reading from the socket,
            # catching zmq.AGAIN and sleeping for 0.1.
            while not self.stop_event.is_set():
                if self.socket.poll(100):
                    # Note that we can still use send_json()/recv_json() here,
                    # the DEALER socket ensures we don't have to deal with
                    # client ids at all.
                    job_id, work = self.socket.recv_json()
                    self.socket.send_json(
                        {'message': 'job_done',
                         'result': self._do_work(work),
                         'job_id': job_id})
        finally:
            self._disconnect()

    def _do_work(self, work):
        result = work['number'] ** 2
        time.sleep(random.randint(1, 10))
        return result











### WHAT FOLLOWS IS NOT THE RIGHT WAY.. JUST FOR REFERENCE

# class Controller(object):
#     """Generate jobs, send the jobs out to workers and collect the results.
#     """
#     def __init__(self, stop_event):
#         self.stop_event = stop_event
#         self.context = zmq.Context()
#         # Create a push socket to send out jobs to the workers
#         self.socket = self.context.socket(zmq.PUSH)
#         self.socket.bind('tcp://127.0.0.1:5755')
#         # And a pull socket to accept the results of the work. In a very
#         # simple case you could let the workers handle the results directly
#         # (for example storing into a database) but this is a little
#         # bit neater.
#         self.socket_result = self.context.socket(zmq.PULL)
#         self.socket_result.bind('tcp://127.0.0.1:5756')

#     def work_iterator(self):
#         """Return an iterator that yields work to be done.

#         This iterator is super boring and yields successive ints using
#         xrange().
#         """
#         return xrange(0, 10000)

#     def run(self):
#         for i in self.work_iterator():
#             # For each job in our list of work send it out to a worker.
#             # Messages sent using a push socket are round-robin load balanced
#             # all connected workers.
#             self.socket.send_json({'number': i})
#             # Poll the incoming socket for results. poll() returns a Truthy
#             # value if there are messages waiting.
#             while self.socket_result.poll(0):
#                 result = self.socket_result.recv_json()
#                 print result['worker_id'], result['result']
#             if self.stop_event.is_set():
#                 break
#         self.stop_event.set()



# class Worker(object):
#         """Accept work in the form of {'number': xxx}, square the number and
#         send it back to the controller in the form
#         {'result': xxx, 'worker_id': yyy}. Our "work" in this case is just
#         squaring the contents of 'number'.
#         """
#         def __init__(self, stop_event):
#             self.stop_event = stop_event
#             self.context = zmq.Context()
#             self.socket = self.context.socket(zmq.PULL)
#             self.socket.connect('tcp://127.0.0.1:5755')
#             self.socket_result = self.context.socket(zmq.PUSH)
#             self.socket_result.connect('tcp://127.0.0.1:5756')
#             # We'll send this id back with our results to make it easier
#             # to verify that work is getting distributed among multiple
#             # workers.
#             self.my_id = uuid.uuid4().hex[:4]

#         def run(self):
#             while not self.stop_event.is_set():
#                 # Poll the socket for incoming messages. This will wait up to
#                 # 0.1 seconds before returning False. The other way to do this is
#                 # is to use zmq.NOBLOCK when reading from the socket, catching
#                 # zmq.AGAIN and sleeping for 0.1.
#                 while self.socket.poll(100):
#                     work = self.socket.recv_json()
#                     self.socket_result.send_json(
#                         {'result': work['number'] ** 2, 'worker_id': self.my_id})
