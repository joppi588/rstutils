Here are some bad cases:

.. |symbol| image:: symbol.png
No blank line after.

.. |empty|

.. |unknown| directive:: symbol.png

.. |invalid 1| there's no directive here
.. |invalid 2| there's no directive here
   With some block quote text, line 1.
   And some more, line 2.

.. |invalid 3| there's no directive here

.. | bad name | bad data

.. |
