import { Routes } from '@angular/router';
import { TvComponent } from './tv.component';
import { PlayComponent } from './play.component';

export const routes: Routes = [
  { path: 'tv', component: TvComponent },
  { path: '', component: PlayComponent },
  { path: '**', redirectTo: '' },
];
