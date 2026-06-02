{% include "default" %}
(allow file-read* (subpath "{{ home }}/.config/gcloud"))
(allow file-write* (subpath "{{ home }}/.config/gcloud"))
(allow file-write* (subpath "{{ home }}/Library/Caches"))
