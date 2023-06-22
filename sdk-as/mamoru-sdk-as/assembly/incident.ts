import { JSONEncoder } from "assemblyscript-json/assembly";
import { encode } from "as-base64/assembly";

export class Incident {
    severity: IncidentSeverity
    message: string
    data: Uint8Array | null
    address: string

    public constructor(severity: IncidentSeverity, message: string, data: Uint8Array | null = null, address: string = "") {
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
                encoder.setString("data", encode(data));
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
