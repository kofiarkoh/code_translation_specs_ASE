#include <stdio.h>
#include <math.h>

int main(){
  int n;
  int a[100];
  int i;
  int s=0;
  float ave;
  float temp;
  float min=101;
  int minindex;

  scanf("%d\n",&n);
  for (i=0;i<n;i++){
    scanf("%d",&a[i]);
    s = s+a[i];
  }
  ave = s/n;
  for (i=0;i<n;i++){
    temp=fabs(ave-a[i]);
    if (min>temp){
      min=temp;
      minindex=i;
    }
  }

  printf("%d",minindex);

  return 0;
}
