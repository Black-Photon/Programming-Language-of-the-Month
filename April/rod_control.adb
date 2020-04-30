with types;

package body rod_control is
    procedure callback (Control_Rods : in out types.Rod_Array; Temperature : types.Kilojoule; Output : types.Kilojoule) is
        use types;
        Diff : Kilojoule;
        Expected : Kilojoule;
    begin
        Diff := Temperature - Last_Temp;
        Diff := Diff * 4; -- Potential temp might increase by as much as 4 times
        Expected := Temperature + Diff;
        if Temperature > 500.000 then
            for i in (Index) loop
                Control_Rods(i) := 1.0;
            end loop;
        elsif Expected > 500.000 then
            for i in (Index) loop
                Control_Rods(i) := 0.8;
            end loop;
        -- Aim to keep at around 400KJ (20KJ/s return)
        elsif Expected > 400.000 then
            for i in (Index) loop
                Control_Rods(i) := 0.6;
            end loop;
        elsif Expected > 250.000 then
            for i in (Index) loop
                Control_Rods(i) := 0.3;
            end loop;
        elsif Expected > 100.000 then
            for i in (Index) loop
                Control_Rods(i) := 0.2;
            end loop;
        end if;
        Last_Temp := Temperature;
    end callback;
begin
    null;
end rod_control;