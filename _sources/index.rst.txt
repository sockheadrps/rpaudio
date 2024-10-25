======================
Rpaudio
======================

*Simple, no-boilerplate audio interface*

.. code-block:: bash

      pip install rpaudio

.. important::

   Rpaudio runs natively on windows. If on Linux or MacOS, refer to `Dependencies Installation`_ section for additional dependencies.

.. toctree::
   :maxdepth: 3
   :hidden:
   
   autoapi/rpaudio/index
   autoapi/rpaudio/effects/index
   autoapi/rpaudio/exceptions/index
   examples/index
   quickstart


`Quickstart Code <./quickstart.html>`_
======================================



Examples
========

.. list-table::
   :widths: 50 50
   :header-rows: 0

   * - **Internal Documentation**
     - **External Resources**
   * - `AudioSink <docs/build/examples/exaudiosink.html>`_
     - `FastAPI Server-side Audio Control <https://github.com/sockheadrps/RpaudioFastAPIExample>`_
   * - `AudioChannel <docs/build/examples/exaudiochannel.html>`_
     - 
   * - `ChannelManager <docs/build/examples/exchannelmanager.html>`_
     - 

 

Dependencies Installation
=========================

.. admonition:: MacOS

   .. code-block:: bash

      brew install gettext
      brew link gettext --force


.. admonition:: Linux 

   Debian/Ubuntu-based distributions

   .. code-block:: bash

      sudo apt-get update
      sudo apt-get install -y pkg-config libasound2-dev

   

   Red Hat/CentOS-based distributions

   .. code-block:: bash

      sudo yum install -y pkg-config alsa-lib-devel

