#include <stdio.h>

int main()
{
  int W, H, n;
  scanf("%d %d %d", &W, &H,&n);
  int X, Y, a;
  int x=0,y=0;
  for(int i=0;i<n;i++){
    scanf("%d %d %d", &X, &Y, &a);
    if(a == 1) x = (X>x) ? X:x;
    if(a == 2) W = (W>X) ? X:W;
    if(a == 3) y = (Y>y) ? Y:y;
    if(a == 4) H = (H>Y) ? Y:H;
  }
  
  int ans = ((W-x)*(H-y) <= 0) ? 0 : (W-x)*(H-y);
  printf("%d", ans);
  return 0;
  
}