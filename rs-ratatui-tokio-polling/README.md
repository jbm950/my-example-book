Example that shows how to use Tokio tasks in conjunction with Ratatui. This
uses the dynamic window project as its base so that it can be shown that the
async code does not lock the interface.

The example uses polling to get events from cross term. This means it's
constantly rerendering.
