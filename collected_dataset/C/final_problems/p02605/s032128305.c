#include <stdio.h>

int main()
{
  int n;
  scanf("%d", &n);
  
  int x[n], y[n], min = 1 << 24, t = 1 << 24, xij, yij;
  char u[n];
	 
  for (int i = 0; i < n; i++)
    scanf("%d %d %c", &x[i], &y[i], &u[i]);
    
  for (int i = 1; i < n; i++)
  {
    for (int j = 0; j < i; j++)
    {
      if (u[i] == u[j]) continue;
      
      xij = x[i] - x[j];
      yij = y[i] - y[j];
      
      if (xij == 0)
      {
        if (yij > 0) u[i] == 'D' && u[j] == 'U' && (t =  yij);
        else 
        if (yij < 0) u[i] == 'U' && u[j] == 'D' && (t = -yij);
      }
      else if (yij == 0)
      {
        if (xij > 0) u[i] == 'L' && u[j] == 'R' && (t =  xij);
        else 
        if (xij < 0) u[i] == 'R' && u[j] == 'L' && (t = -xij);
      }
      else if (xij == yij)
      {
        if (xij > 0) 
          (u[i] == 'D' && u[j] == 'R' || u[i] == 'L' && u[j] == 'U') && (t =  xij << 1);
        else if (xij < 0)
          (u[i] == 'U' && u[j] == 'L' || u[i] == 'R' && u[j] == 'D') && (t = -xij << 1);
      }
      else if (xij == -yij)
      {
        if (xij > 0) 
          (u[i] == 'U' && u[j] == 'R' || u[i] == 'L' && u[j] == 'D') && (t = xij << 1);
        else if (xij < 0) 
          (u[i] == 'D' && u[j] == 'L' || u[i] == 'R' && u[j] == 'U') && (t = yij << 1);
      }
             
      t < min && (min = t) || (t = min);
    }
  }
  
  if (min >= 1 << 24)
    printf("SAFE");
  else
    printf("%d", min * 5);
  
  return 0;
}
