/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from '@tauri-apps/api/core';
import {
  disable_autostart,
  enable_autostart,
  is_autostart_enabled,
} from '@opus-api/autostart';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  greetingMessage = '';
  autostartEnabled = '';

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

    invoke<string>('plugin:state|greet', { name }).then((text) => {
      this.greetingMessage = text;
    });
  }

  init_connection(event: SubmitEvent, addr: string, port: string): void {
    event.preventDefault();

    invoke('plugin:grpc|init_connection', { addr, port }).then(() => {});
  }

  async get_autostart_enabled(event: SubmitEvent): Promise<void> {
    event.preventDefault();

    this.autostartEnabled =
      (await is_autostart_enabled()) ? 'Enabled' : 'Disabled';

    // test autostart
    (await is_autostart_enabled()) ? disable_autostart() : enable_autostart();
  }
}
