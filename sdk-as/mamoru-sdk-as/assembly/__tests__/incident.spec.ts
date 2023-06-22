import { Incident, IncidentSeverity } from "../incident";

describe("Incident serialization", () => {
    test("minimal incident", () => {
        const incident = new Incident(IncidentSeverity.Alert, "Hello, I am a mine turtle!");
        const json = incident.toJSON();

        expect(json).toBe('{"severity":"alert","message":"Hello, I am a mine turtle!"}');
    });
    test("an incident with data", () => {
        let data = new Uint8Array(4);
        data.set([0, 1, 2, 3]);
        const incident = new Incident(IncidentSeverity.Alert, "An incident", data);
        const json = incident.toJSON();

        expect(json).toBe('{"severity":"alert","message":"An incident","data":"AAECAw=="}');
    });
});
