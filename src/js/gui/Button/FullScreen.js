// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//


/******************************************************************************
 * Aladin Lite project
 *
 * File gui/Stack/Menu.js
 *
 *
 * Author: Matthieu Baumann [CDS, matthieu.baumann@astro.unistra.fr]
 *
 *****************************************************************************/

import { ActionButton } from "../Widgets/ActionButton.js";
import { ALEvent } from "../../events/ALEvent";


import restoreIcon from './../../../../assets/icons/restore.svg';
import maximizeIcon from './../../../../assets/icons/maximize.svg';

export class FullScreenActionButton extends ActionButton {
    // Constructor
    constructor(aladin, options) {
        let self;
        super({
            icon: {
                size: 'medium',
                monochrome: true,
                url: aladin.isInFullscreen ? restoreIcon : maximizeIcon
            },
            classList: ['aladin-fullScreen-control'],
            ...options,
            tooltip: {
                content: aladin.isInFullscreen ? 'Restore original size' : 'Full-screen',
                position: {
                    direction: 'left'
                }
            },
            action(e) {
                if (aladin.statusBar) {
                    aladin.statusBar.removeMessage('tooltip')
                }
    
                aladin.toggleFullscreen(aladin.options.realFullscreen);    
            }
        })

        self = this;

        // Listen to the fullscreen toggled event to change the icon of the button
        ALEvent.FULLSCREEN_TOGGLED.listenedBy(aladin.aladinDiv, (event) => {
            const isInFullscreen = event.detail.fullscreen;

            if (isInFullscreen) {
                self.update({
                    icon: {
                        size: 'medium',
                        monochrome: true,
                        url: restoreIcon
                    },
                    tooltip: {
                        content: 'Restore original size',
                        position: {
                            direction: 'left'
                        }
                    }
                });
            } else {
                self.update({
                    icon: {
                        size: 'medium',
                        monochrome: true,
                        url: maximizeIcon
                    },
                    tooltip: {
                        content: 'Fullscreen',
                        position: {
                            direction: 'left'
                        }
                    }
                });
            }
        });
    }
}