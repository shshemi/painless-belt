{% include "default" %}
(allow file-read* (subpath "{{ home }}/.aws"))
(allow file-write* (subpath "{{ home }}/.aws"))
(allow file-write* (subpath "{{ home }}/Library/Caches"))
