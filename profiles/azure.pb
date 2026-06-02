{% include "default" %}
(allow file-read* (subpath "{{ home }}/.azure"))
(allow file-write* (subpath "{{ home }}/.azure"))
(allow file-write* (subpath "{{ home }}/Library/Caches"))
