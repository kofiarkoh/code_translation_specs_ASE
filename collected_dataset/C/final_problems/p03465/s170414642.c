#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;
typedef unsigned long long int uint64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

void run(void){
  int n;
  scanf("%d",&n);
  int *a=(int *)malloc(sizeof(int)*n);
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  int sum=0;
  for(i=0;i<n;i++) sum+=a[i];

  const int len=(sum+63)/64;
  uint64 *bit=(uint64 *)calloc(len,sizeof(uint64));
  bit[0]=1;
  int localSum=0;
  for(i=0;i<n;i++){
    const int aa=a[i];
    int j;
    for(j=(localSum+63)/64;j>=0;j--){
      bit[(64*j+aa)>>6]|=bit[j]<<(aa&0x3F);
      if(aa%64!=0){
	bit[((64*j+aa)>>6)+1]|=bit[j]>>(aa&0x3F);
      }
    }
    localSum+=aa;
  }
  int median=(sum+1)/2;
  while(((bit[median>>6]>>(median&0x3F))&0x01)==0) median++;
  printf("%d\n",median);
}

int main(void){
  run();
  return 0;
}
