# Context hook extension
# This package also provides a convenient extension class allowing template writers to update the context used to render templates, 
# in order to add, modify or remove items of the context.

# In one of your relative path extensions modules, create a class that inherits from ContextHook, and override its hook method:

# from copier_template_extensions import ContextHook


# class ContextUpdater(ContextHook):
#     def hook(self, context):
#         context["say"] = "hello " + context["name"]
# In your Jinja templates, you will now have access to the {{ say }} variable directly.

# This can be extremely useful in template projects where you don't want to ask too many questions to the users and instead 
# infer some values from their answers.

from copier_template_extensions import ContextHook



class ContextUpdater(ContextHook):
    def hook(self, context):
        context['say'] = 'hello' + context['name']