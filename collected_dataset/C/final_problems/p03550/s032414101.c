#include <stdio.h>
#include <stdlib.h>
int main() {
  int n,z,w,a[2000],i;
  scanf("%d %d %d",&n,&z,&w);
  for (i = 0; i < n; i++) {
    scanf("%d",&a[i]);
  }
  if (abs(a[n-1]-w)<abs(a[n-2]-a[n-1])) {
    printf("%d\n",abs(a[n-2]-a[n-1]));
  } else {
    printf("%d\n",abs(a[n-1]-w));
  }
  return 0;
}