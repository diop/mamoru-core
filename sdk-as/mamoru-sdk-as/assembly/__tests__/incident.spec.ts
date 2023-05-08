import { Incident, IncidentDataStruct, IncidentSeverity, NumberDataValue, StructDataValue } from "../incident";
import { JSONEncoder } from "assemblyscript-json/assembly";

describe("Incident serialization", () => {
    test("minimal incident", () => {
        const incident = new Incident(IncidentSeverity.Alert, "Hello, I am a mine turtle!");
        const json = incident.toJSON();

        expect(json).toBe('{"severity":"alert","message":"Hello, I am a mine turtle!"}');
    });
    test("an incident with data", () => {
        let data = new IncidentDataStruct();
        data.addString("hello", "I am a mine turtle");

        const incident = new Incident(IncidentSeverity.Alert, "An incident", data);
        const json = incident.toJSON();

        expect(json).toBe('{"severity":"alert","message":"An incident","data":{"hello":"I am a mine turtle"}}');
    });
});

describe("IncidentDataStruct serialization", () => {
    test("null serialization", () => {
        let data = new IncidentDataStruct();
        data.addNull("null_field");

        const json = dataToJson(data);

        expect(json).toBe('{"null_field":null}');
    });
    test("number serialization", () => {
        let data = new IncidentDataStruct();
        data.addNumber("number_field", 42);

        const json = dataToJson(data);

        expect(json).toBe('{"number_field":42.0}');
    });
    test("string serialization", () => {
        let data = new IncidentDataStruct();
        data.addString("string_field", "value");

        const json = dataToJson(data);

        expect(json).toBe('{"string_field":"value"}');
    });
    test("boolean serialization", () => {
        let data = new IncidentDataStruct();
        data.addBoolean("boolean_field", false);

        const json = dataToJson(data);

        expect(json).toBe('{"boolean_field":false}');
    });
    test("struct serialization", () => {
        let nestedData = new IncidentDataStruct();
        nestedData.addString("hello", "world");

        let data = new IncidentDataStruct();
        data.addStruct("nested_struct", nestedData);

        const json = dataToJson(data);

        expect(json).toBe('{"nested_struct":{"hello":"world"}}');
    });
    test("list serialization", () => {
        let data1 = new IncidentDataStruct();
        data1.addString("hello", "world");

        let data2 = new IncidentDataStruct();
        data2.addList("list", [
            new NumberDataValue(42),
            new StructDataValue(data1),
        ]);

        const json = dataToJson(data2);

        expect(json).toBe('{"list":[42.0,{"hello":"world"}]}');
    });
});

function dataToJson(data: IncidentDataStruct): string {
    let encoder = new JSONEncoder();

    encoder.pushObject(null);
    data.addToJson(encoder);
    encoder.popObject();

    return encoder.toString();
}
