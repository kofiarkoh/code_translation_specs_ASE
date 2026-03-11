#include <stdio.h>

int main() {
  double d, t, s;
  
  scanf("%lf %lf %lf", &d, &t, &s);
  
  if(d/s <= t) {
    printf("Yes");
  }
  else {
    printf("No");
  }
  
  return (0);
}