with types;

package rod_control is
    procedure callback (Control_Rods : in out types.Rod_Array; Temperature : types.Kilojoule; Output : types.Kilojoule);
private
    Last_Temp : types.Kilojoule := 0.000;
end rod_control;