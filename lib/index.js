"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Menu = exports.getDefaultConfig = void 0;
var PopupMenu = __importStar(require("../build/index"));
var UUID = "MenuItem";
var getDefaultConfig = function () {
    return PopupMenu.getDefaultConfig();
};
exports.getDefaultConfig = getDefaultConfig;
var Menu = /** @class */ (function () {
    function Menu() {
        this.hwnd = 0;
        this.type = "";
        this.callbacks = {};
        this.uuid = 0;
    }
    Menu.prototype.ready = function () {
        if (!this.hwnd)
            throw new Error("Menu does not exist");
    };
    Menu.prototype.getWindowHandle = function () {
        return this.hwnd;
    };
    Menu.prototype.buildFromTemplate = function (hwnd, template) {
        var effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplate(hwnd, effectiveTemplate);
    };
    Menu.prototype.buildFromTemplateWithTheme = function (hwnd, template, theme) {
        var effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplateWithTheme(hwnd, effectiveTemplate, theme);
    };
    Menu.prototype.buildFromTemplateWithConfig = function (hwnd, template, config) {
        var effectiveTemplate = this.toEffectiveTemplates(template);
        this.hwnd = PopupMenu.buildFromTemplateWithConfig(hwnd, effectiveTemplate, config);
    };
    Menu.prototype.toEffectiveTemplates = function (items) {
        var _this = this;
        return items.map(function (item) {
            var newItem = _this.toEffectiveTemplate(item);
            if (newItem.type == "submenu" && newItem.submenu) {
                _this.toEffectiveTemplates(newItem.submenu);
            }
            return newItem;
        });
    };
    Menu.prototype.toEffectiveTemplate = function (item) {
        item.id = this.getId(item.id);
        item.type = this.getType(item.type, item.submenu);
        if (item.click) {
            this.callbacks[item.id] = item.click;
        }
        else {
            this.callbacks[item.id] = function () { };
        }
        return item;
    };
    Menu.prototype.getId = function (id) {
        if (!id) {
            this.uuid++;
            return UUID + this.uuid;
        }
        return id;
    };
    Menu.prototype.getType = function (type, submenu) {
        if (!type) {
            return submenu ? "submenu" : "normal";
        }
        return type;
    };
    Menu.prototype.toMenuItem = function (item) {
        var submenu = new Menu();
        if (item.submenu && Object.keys(item.submenu).length) {
            submenu.hwnd = item.submenu.hwnd;
            submenu.type = item.submenu.type;
        }
        return __assign(__assign({}, item), { click: this.callbacks[item.id], submenu: submenu });
    };
    Menu.prototype.popup = function (x, y) {
        return __awaiter(this, void 0, void 0, function () {
            var result;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        this.ready();
                        return [4 /*yield*/, PopupMenu.popup(this.hwnd, x, y)];
                    case 1:
                        result = _a.sent();
                        console.log(result);
                        if (Object.keys(result).length) {
                            this.callbacks[result.id](result);
                        }
                        return [2 /*return*/];
                }
            });
        });
    };
    Menu.prototype.items = function () {
        var _this = this;
        this.ready();
        return PopupMenu.items(this.hwnd).map(function (item) { return _this.toMenuItem(item); });
    };
    Menu.prototype.remove = function (item) {
        this.ready();
        PopupMenu.remove(this.hwnd, item);
    };
    Menu.prototype.removeAt = function (index) {
        this.ready();
        PopupMenu.removeAt(this.hwnd, index);
    };
    Menu.prototype.append = function (item) {
        this.ready();
        PopupMenu.append(this.hwnd, this.toEffectiveTemplate(item));
    };
    Menu.prototype.insert = function (index, item) {
        this.ready();
        PopupMenu.insert(this.hwnd, index, this.toEffectiveTemplate(item));
    };
    Menu.prototype.setTheme = function (theme) {
        this.ready();
        PopupMenu.setTheme(this.hwnd, theme);
    };
    Menu.prototype.getMenuItemById = function (id) {
        this.ready();
        var item = PopupMenu.getMenuItemById(this.hwnd, id);
        if (item) {
            return this.toMenuItem(item);
        }
        return item;
    };
    return Menu;
}());
exports.Menu = Menu;
