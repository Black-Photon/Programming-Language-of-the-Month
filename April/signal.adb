package body signal is
   protected body Handler is
      entry Wait when Call_Count > 0 is
      begin
         Call_Count := Call_Count - 1;
      end Wait;

      procedure Handle is
      begin
         Call_Count := Call_Count + 1;
      end Handle;
 
   end Handler;
 
end signal;