/******************************************************************************
 * Aladin Lite project
 * 
 * File MOC
 *
 * This class represents a MOC (Multi Order Coverage map) layer
 * 
 * Author: Thomas Boch[CDS], Matthieu Baumann[CDS]
 * 
 *****************************************************************************/

import { Aladin }   from "./Aladin.js";
import { Utils }   from "./Utils";
import { Color } from "./Color.js";

import { ALEvent } from "./events/ALEvent.js";

/**
* @typedef {Object} MOCOptions
* @description Options for configuring a MOC (Multi-Order-Coverage).
*
* @property {string} [name="MOC"] - The name of the catalog.
* @property {string} [color] - The color of the MOC HEALPix cell edges. Is used only if `perimeter` is true or `edge` is true
* @property {string} [fillColor] - The filling of the MOC.
* @property {string} [fill=false] - Draw the filled MOC with `options.fillColor`.
* @property {Boolean} [perimeter=false] - Draw the perimeter of the MOC only with `options.color`.
* @property {string} [edge=!fill && !perimeter] - Draw the edges of the HEALPix cells with `options.color`.
    The HEALPix cell edges compositing the MOC will be drawn if `fill` and `perimeter` are false
* @property {number} [lineWidth=3] - The line width in pixels 
* @property {number} [opacity=1.0] - The opacity of the colors
*/

export let MOC = (function() {
    /**
     * Represents a Multi-Order-Coverage with configurable options for display and interaction.
     *
     * @class
     * @constructs MOC
     * @param {MOCOptions} options - Configuration options for the MOC
     */
    let MOC = function(options) {
        //this.order = undefined;

        this.uuid = Utils.uuidv4();
        this.type = 'moc';

        // TODO homogenize options parsing for all kind of overlay (footprints, catalog, MOC)
        options = options || {};
        this.name = options.name || "MOC";

        // User can change the opacity, color, lineWidth and fillColor properties
        this._defineProperties()

        this.color = options.color || Color.getNextColor();
        this.color = Color.standardizeColor(this.color);

        this.fillColor = options.fillColor || this.color;
        this.fillColor = Color.standardizeColor(this.fillColor);

        if (options && options.perimeter) {
            this.perimeter = true;
        } else {
            this.perimeter = false;
        }

        if (options && options.opacity !== undefined) {
            this.opacity = options.opacity;
        } else {
            this.opacity = 1.0;
        }
        
        if (options && options.fill) {
            this.fill = true;
        } else {
            this.fill = false;
        }

        if (options && options.edge) {
            this.edge = true;
        } else {
            this.edge = false;
        }

        if (!this.fill && !this.perimeter && !this.edge) {
            this.edge = true;
        }

        this.opacity = Math.max(0, Math.min(1, this.opacity)); // 0 <= this.opacity <= 1
        this.lineWidth = options["lineWidth"] || 3;

        //this.proxyCalled = false; // this is a flag to check whether we already tried to load the MOC through the proxy

        this.isShowing = true;
        this.ready = false;
        this.skyFrac = undefined;
    }

    /**
     *  Return a value between 0 and 1 denoting the fraction of the sky
     *  covered by the MOC
     * 
     * @memberof MOC
     * 
     * @returns {number} The sky fraction covered by the MOC 
     */
    MOC.prototype.skyFraction = function() {
        return this.skyFrac;
    };

    /**
     * set MOC data by parsing a MOC serialized in JSON
     * (as defined in IVOA MOC document, section 3.1.1)
     */
    MOC.prototype.parse = function(data, successCallback, errorCallback) {
        if (typeof data === 'string' || data instanceof String) {
            let url = data;
            this.promiseFetchData = Utils.fetch({
                url,
                method: 'GET',
                dataType: 'blob',
                error: function(err) {
                    console.log('Something went wrong: ' + err);
                }
            }).then(blob => blob.arrayBuffer());
        } else {
            this.promiseFetchData = Promise.resolve(data)
        }

        this.successCallback = successCallback;
        this.errorCallback = errorCallback;
    };

    MOC.prototype.setView = function(view, idx) {
        let self = this;

        this.view = view;
        this.mocParams = new Aladin.wasmLibs.core.MOCOptions(this.uuid, this.opacity, this.lineWidth, this.perimeter, this.fill, this.edge, this.isShowing, this.color, this.fillColor);

        this.promiseFetchData
            .then((data) => {
                if (data instanceof ArrayBuffer) {
                    // from an url
                    const buf = data;
                    self.view.wasm.addFITSMOC(self.mocParams, new Uint8Array(buf));
                } else if(data.ra && data.dec && data.radius) {
                    // circle
                    const c = data;
                    self.view.wasm.addConeMOC(self.mocParams, c.ra, c.dec, c.radius);
                } else if(data.ra && data.dec) {
                    // polygon
                    const p = data;
                    self.view.wasm.addPolyMOC(self.mocParams, p.ra, p.dec);
                } else {
                    // json moc
                    self.view.wasm.addJSONMoc(self.mocParams, data);
                }

                // Add the fetched moc to the rust backend
                self.ready = true;

                if (self.successCallback) {
                    self.successCallback(self)
                }

                // Cache the sky fraction
                self.skyFrac = self.view.wasm.getMOCSkyFraction(this.uuid);

                // Add it to the view
                self.view.mocs.push(self);
                self.view.insertOverlay(self, idx);

                // Tell the MOC has been fully loaded and can be sent as an event
                ALEvent.GRAPHIC_OVERLAY_LAYER_ADDED.dispatchedTo(self.view.aladinDiv, {layer: self});

                self.view.requestRedraw();
            })
            .catch(e => {
                console.error('MOC load error:' + e)
                if (self.errorCallback)
                    self.errorCallback(self);
            })
    };

    MOC.prototype._defineProperties = function() {
        Object.defineProperties(this, {
            opacity: {
                get() {
                    return this._opacity;
                },
                set(opacity) {
                    this._opacity = opacity;
                    this.reportChange();
                }
            },
            color: {
                get() {
                    return this._color;
                },
                set(color) {
                    this._color = Color.standardizeColor(color);
                    this.reportChange();
                }
            },
            fillColor: {
                get() {
                    return this._fillColor;
                },
                set(fillColor) {
                    this._fillColor = Color.standardizeColor(fillColor);
                    this.reportChange();
                }
            },
            lineWidth: {
                get() {
                    return this._lineWidth;
                },
                set(lineWidth) {
                    this._lineWidth = lineWidth;
                    this.reportChange();
                }
            },
        });
    };

    MOC.prototype.reportChange = function() {
        if (this.view) {
            // update the new moc params to the backend
            this.mocParams = new Aladin.wasmLibs.core.MOCOptions(this.uuid, this.opacity, this.lineWidth, this.perimeter, this.fill, this.edge, this.isShowing, this.color, this.fillColor);
            this.view.wasm.setMocParams(this.mocParams);

            this.view.requestRedraw();
        }
    };

    MOC.prototype.delete = function() {
        if (this.view) {
            // update the new moc params to the backend
            this.view.wasm.removeMoc(this.uuid);
            this.view.requestRedraw();
        }
    };

    /**
     * Show the MOC object 
     *
     * @memberof MOC
     */
    MOC.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        this.reportChange();
    };

    /**
     * Hide the MOC object 
     *
     * @memberof MOC
     */
    MOC.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        this.reportChange();
    };

    /**
     * Tests whether a given (ra, dec) point on the sky is within the current MOC object
     *
     * @memberof MOC
     * @param {number} ra - Right-Ascension of the location in degrees and ICRS frame
     * @param {number} dec - Declination of the location in degrees and ICRS frame
     *
     * @returns {boolean} True if the point is contained, false otherwise
     */
    MOC.prototype.contains = function(ra, dec) {
        if (!this.ready) {
            throw this.name + " is not yet ready, either because it has not been downloaded yet or because it has not been added to the aladin instance."
        }

        // update the new moc params to the backend
        return this.view.wasm.mocContains(this.uuid, ra, dec);
    };

     /**
     * Serialize a MOC into different format
     *
     * @memberof MOC
     * @param {string} [format='json'] - The output format. Only `json` is currently supported but `fits` could be added.
     */
    MOC.prototype.serialize = function(format) {
        if (!this.ready) {
            throw this.name + " is not yet ready, either because it has not been downloaded yet or because it has not been added to the aladin instance."
        }

        return this.view.wasm.mocSerialize(this.uuid, format);
    }

    return MOC;

})();

    
