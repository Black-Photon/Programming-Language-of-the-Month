with types;

function get_rod_absorption (a : types.Rod_Array) return types.Percent is
    use types;
    Sum : Percent;
begin
    Sum := 0.0;
    for i in (Index) loop
        Sum := Sum + a(i);
    end loop;
    return Sum / 100.0;
end get_rod_absorption;