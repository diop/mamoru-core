import { JSONEncoder } from "assemblyscript-json/assembly";

export class Incident {
    severity: IncidentSeverity
    message: string
    data: IncidentDataStruct | null
    address: string

    public constructor(severity: IncidentSeverity, message: string, data: IncidentDataStruct | null = null, address: string = "") {
        this.severity = severity;
        this.message = message;
        this.data = data;
        this.address = address;
    }

    toJSON(): string {
        let encoder = new JSONEncoder();

        {
            encoder.pushObject(null);
            encoder.setString("severity", incidentSeverityToString(this.severity));
            encoder.setString("message", this.message);

            if (this.address != "") {
                encoder.setString("address", this.address);
            }

            const data = this.data;

            if (data != null) {
                encoder.pushObject("data");
                data.addToJson(encoder);
                encoder.popObject();
            }

            encoder.popObject();
        }

        return encoder.toString();
    }
}

export enum IncidentSeverity {
    Info,
    Warning,
    Error,
    Alert,
}

function incidentSeverityToString(severity: IncidentSeverity): string {
    switch (severity) {
        case IncidentSeverity.Info:
            return "info"
        case IncidentSeverity.Warning:
            return "warning"
        case IncidentSeverity.Error:
            return "error"
        case IncidentSeverity.Alert:
            return "alert"
        default:
            return "UNDEFINED"
    }
}

/**
 * The object to put custom data into an {Incident}.
 * See public methods for available data types.
 *
 * @example
 * data.addNull("null");
 * data.addNumber("number", 42.0);
 * data.addString("string", "hello");
 * data.addBoolean("boolean", true);
 * data.addList("list", [
 *     new StringDataValue("first"),
 *     new StringDataValue("second"),
 * ]);
 *
 * // assert data == {"null": null, "number": 42.0, "string": "hello", "boolean": true, "list": ["first", "second"]}
 */
export class IncidentDataStruct {
    fields: Map<string, IncidentDataValue>

    constructor() {
        this.fields = new Map<string, IncidentDataValue>();
    }

    public addNull(field_name: string): boolean {
        return this.addField(field_name, new NullDataValue());
    }

    public addNumber(field_name: string, value: number): boolean {
        return this.addField(field_name, new NumberDataValue(value));
    }

    public addString(field_name: string, value: string): boolean {
        return this.addField(field_name, new StringDataValue(value));
    }

    public addBoolean(field_name: string, value: boolean): boolean {
        return this.addField(field_name, new BooleanDataValue(value));
    }

    public addStruct(field_name: string, value: IncidentDataStruct): boolean {
        return this.addField(field_name, new StructDataValue(value));
    }

    public addList(field_name: string, value: Array<IncidentDataValue>): boolean {
        return this.addField(field_name, new ListDataValue(value));
    }

    addField(field_name: string, data: IncidentDataValue): boolean {
        if (this.fields.has(field_name)) {
            return false;
        } else {
            this.fields.set(field_name, data);

            return true;
        }
    }

    addToJson(encoder: JSONEncoder): void {
        const keys = this.fields.keys();

        for (let i = 0; i < keys.length; i++) {
            const key = keys[i];

            this.fields.get(key).addToJson(encoder, key);
        }
    }

}

export abstract class IncidentDataValue {
    abstract addToJson(encoder: JSONEncoder, key: string | null): void;
}

export class NullDataValue extends IncidentDataValue {
    addToJson(encoder: JSONEncoder, key: string | null): void {
        encoder.setNull(key);
    }
}

export class NumberDataValue extends IncidentDataValue {
    value: number

    constructor(value: number) {
        super();

        this.value = value;
    }

    addToJson(encoder: JSONEncoder, key: string | null): void {
        encoder.setFloat(key, this.value);
    }
}

export class StringDataValue extends IncidentDataValue {
    value: string

    constructor(value: string) {
        super();

        this.value = value;
    }

    addToJson(encoder: JSONEncoder, key: string | null): void {
        encoder.setString(key, this.value);
    }
}

export class BooleanDataValue extends IncidentDataValue {
    value: boolean

    constructor(value: boolean) {
        super();

        this.value = value;
    }

    addToJson(encoder: JSONEncoder, key: string | null): void {
        encoder.setBoolean(key, this.value);
    }
}

export class StructDataValue extends IncidentDataValue {
    value: IncidentDataStruct

    constructor(value: IncidentDataStruct) {
        super();

        this.value = value;
    }

    addToJson(encoder: JSONEncoder, key: string | null): void {
        encoder.pushObject(key);
        this.value.addToJson(encoder);
        encoder.popObject();
    }
}

export class ListDataValue extends IncidentDataValue {
    value: Array<IncidentDataValue>

    constructor(value: Array<IncidentDataValue>) {
        super();

        this.value = value;
    }

    addToJson(encoder: JSONEncoder, key: string | null): void {
        encoder.pushArray(key);

        for (let i = 0; i < this.value.length; i++) {
            this.value[i].addToJson(encoder, null);
        }

        encoder.popArray();
    }
}
