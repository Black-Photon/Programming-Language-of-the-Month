with types;

procedure rod_control (Control_Rods : in out types.Rod_Array; Temperature : types.Kilojoule; Output : types.Kilojoule) is
    use types;
begin
    if Temperature > 500.000 then
        for i in (Index) loop
            Control_Rods(i) := 1.0;
        end loop;
    end if;
end rod_control;