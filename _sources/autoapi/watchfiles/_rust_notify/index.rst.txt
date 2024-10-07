watchfiles._rust_notify
=======================

.. py:module:: watchfiles._rust_notify






Module Contents
---------------

.. py:exception:: WatchfilesRustInternalError

   Bases: :py:obj:`RuntimeError`


   Raised when RustNotify encounters an unknown error.

   If you get this a lot, please check [github](https://github.com/samuelcolvin/watchfiles/issues) issues
   and create a new issue if your problem is not discussed.


.. py:class:: RustNotify(watch_paths, debug, force_polling, poll_delay_ms, recursive, ignore_permission_denied)

   Interface to the Rust [notify](https://crates.io/crates/notify) crate which does
   the heavy lifting of watching for file changes and grouping them into events.


   .. py:method:: __enter__()

      Does nothing, but allows `RustNotify` to be used as a context manager.

      !!! note

          The watching thead is created when an instance is initiated, not on `__enter__`.



   .. py:method:: __exit__(*args)

      Calls [`close`][watchfiles._rust_notify.RustNotify.close].



   .. py:method:: close()

      Stops the watching thread. After `close` is called, the `RustNotify` instance can no
      longer be used, calls to [`watch`][watchfiles._rust_notify.RustNotify.watch] will raise a `RuntimeError`.

      !!! note

          `close` is not required, just deleting the `RustNotify` instance will kill the thread
          implicitly.

          As per [#163](https://github.com/samuelcolvin/watchfiles/issues/163) `close()` is only required because
          in the event of an error, the traceback in `sys.exc_info` keeps a reference to `watchfiles.watch`'s
          frame, so you can't rely on the `RustNotify` object being deleted, and thereby stopping
          the watching thread.



   .. py:method:: watch(debounce_ms, step_ms, timeout_ms, stop_event)

      Watch for changes.

      This method will wait `timeout_ms` milliseconds for changes, but once a change is detected,
      it will group changes and return in no more than `debounce_ms` milliseconds.

      The GIL is released during a `step_ms` sleep on each iteration to avoid
      blocking python.

      :param debounce_ms: maximum time in milliseconds to group changes over before returning.
      :param step_ms: time to wait for new changes in milliseconds, if no changes are detected
                      in this time, and at least one change has been detected, the changes are yielded.
      :param timeout_ms: maximum time in milliseconds to wait for changes before returning,
                         `0` means wait indefinitely, `debounce_ms` takes precedence over `timeout_ms` once
                         a change is detected.
      :param stop_event: event to check on every iteration to see if this function should return early.
                         The event should be an object which has an `is_set()` method which returns a boolean.

      :returns: See below.

      Return values have the following meanings:

      * Change details as a `set` of `(event_type, path)` tuples, the event types are ints which match
        [`Change`][watchfiles.Change], `path` is a string representing the path of the file that changed
      * `'signal'` string, if a signal was received
      * `'stop'` string, if the `stop_event` was set
      * `'timeout'` string, if `timeout_ms` was exceeded



