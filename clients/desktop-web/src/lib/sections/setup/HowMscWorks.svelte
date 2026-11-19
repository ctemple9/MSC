<section class="how-msc-works" aria-label="How MSC works">
  <h2>How does MSC work?</h2>
  <ol class="architecture">
    <li>
      <h3>The engine runs your servers</h3>
      <p>Wherever the engine runs, that’s where Minecraft runs.</p>
    </li>
    <li>
      <h3>This app controls the engine</h3>
      <p>
        Manage an engine on this computer or another one. The terminal interface (TUI) is another
        way to control it.
      </p>
    </li>
    <li>
      <h3>Closing this window doesn’t stop your servers</h3>
      <p>Like putting away a TV remote while the TV stays on.</p>
    </li>
  </ol>
  <details class="service-details">
    <summary>More about the engine</summary>
    <div class="expanded">
      <p class="detail">
        The background service, also called the engine, starts Minecraft and manages your servers,
        worlds, and files. Wherever the engine is running, that’s where your Minecraft servers run,
        whether on this computer or another one.
      </p>
      <p class="detail">
        Use this window to create servers, change settings, and see what’s happening. The app is
        separate from the engine, so you can control it whether it’s running on this computer or
        another one. If you prefer working in a terminal, MSC’s text interface (TUI) is another
        remote for the same engine.
      </p>
      <p class="detail">
        Closing this window disconnects the control panel. The engine and your Minecraft servers
        keep running, like putting away a TV remote while the TV stays on.
      </p>
    </div>
  </details>
  <div class="connection-explanation">
    <p>
      To stop a Minecraft server, click <strong>Stop</strong>. To stop the engine, click
      <strong>Stop Engine</strong>. Or turn off the computer.
    </p>
  </div>
</section>

<style>
  .how-msc-works {
    font-size: 13px;
    line-height: 1.5;
  }
  h2 {
    font-size: 22px;
    font-weight: 500;
    margin: 0 0 22px;
  }
  .architecture {
    list-style: none;
    padding: 0;
    margin: 0;
    display: grid;
    gap: 22px;
  }
  h3 {
    font-size: 14px;
    font-weight: 500;
    margin: 0 0 5px;
  }
  p {
    margin: 0;
    color: var(--msc-text-secondary);
  }
  .service-details {
    margin-top: 22px;
  }
  summary {
    cursor: pointer;
    color: var(--msc-text-secondary);
  }
  .expanded {
    display: grid;
    gap: 12px;
    padding-top: 12px;
  }
  .connection-explanation {
    margin-top: 22px;
    font-size: 12px;
  }
</style>
