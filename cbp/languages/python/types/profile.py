from __future__ import annotations
import strawberry as sb


@sb.type
class Profile:
    duration:   float
    cpu:        float
    mem:        float

