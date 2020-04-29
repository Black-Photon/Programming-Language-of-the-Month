with types;

procedure rod_control (Control_Rods : in out types.Rod_Array; Temperature : types.Kilojoule; Output : types.Kilojoule) is
    use types;
begin
    if Temperature > 400.000 then
        for i in (Index) loop
            Control_Rods(i) := 1.0;
        end loop;
    elsif Temperature > 100.000 then
        for i in (Index) loop
            Control_Rods(i) := 0.5;
        end loop;
    end if;
end rod_control;