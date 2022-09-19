/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { EventGuid } from './EventGuid';
import type { EventKey } from './EventKey';
import type { MoveType } from './MoveType';
import type { U64 } from './U64';

/**
 * An event from a transaction
 */
export type Event = {
    key: EventKey;
    guid: EventGuid;
    sequence_number: U64;
    type: MoveType;
    /**
     * The JSON representation of the event
     */
    data: any;
};
