{% include "node" %}
(allow file-read* (subpath "{{ home }}/.codex"))
(allow file-read* (subpath "{{ home }}/Library/Keychains"))
(allow file-write* (subpath "{{ home }}/.codex"))
