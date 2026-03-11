#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

typedef struct top2{
  int a,b;
} val;

void max(val *x,val *a,val *b,int *array){
  int i,j,k,l;
  i=a->a;
  j=a->b;
  k=b->a;
  l=b->b;
  x->a=array[i]>=array[k]?i:k;
  if(i==k){
    x->b=array[j]>=array[l]?j:l;
    return;
  }
  if(array[i]<array[k]){
    k=i;
  }
  k=array[k]>=array[l]?k:l;
  x->b=array[j]>=array[k]?j:k;
  return;
}

void run(void){
  int n;
  scanf("%d",&n);
  int len=1<<n;
  int *a=(int *)malloc(sizeof(int)*len);
  int i;
  for(i=0;i<len;i++) scanf("%d",a+i);
  val *array=(val *)malloc(sizeof(val)*len);
  for(i=0;i<len;i++){
    array[i].a=a[0]>=a[i]?0:i;
    array[i].b=array[i].a^i;
  }
  for(i=1;i<len;i++){
    int j;
    for(j=0;(1<<j)<len;j++){
      if(((i>>j)&0x01)==1 && (i^(1<<j))!=0){
	max(array+i,array+i,array+(i^(1<<j)),a);
      }
    }
  }
  int max=0;
  for(i=1;i<len;i++){
    max=MAX(max,a[array[i].a]+a[array[i].b]);
    printf("%d\n",max);
  }
  return;
}

int main(void){
  run();
  return 0;
}
