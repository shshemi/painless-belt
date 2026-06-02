{% include "node" %}
(allow file-read* (subpath "{{ home }}/.gemini"))
(allow file-read* (subpath "{{ home }}/Library/Keychains"))
(allow file-write* (subpath "{{ home }}/.gemini"))
