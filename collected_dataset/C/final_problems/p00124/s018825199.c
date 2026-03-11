#include <stdio.h>

typedef struct{
  char name[21];
  int p;
}data;

void solve(int n,data a[]){
  int w,l,d,i,j;
  data t;
  for(i=0;i<n;i++){
    scanf("%s %d %d %d",a[i].name,&w,&l,&d);
    a[i].p=w*3+d;
  }
  for(i=0;i<n-1;i++){
    for(j=n-1;j>i;j--){
      if(a[j-1].p<a[j].p){
	t=a[j];
	a[j]=a[j-1];
	a[j-1]=t;
      }
    }
  }
  for(i=0;i<n;i++)
    printf("%s,%d\n",a[i].name,a[i].p);
}

int main(void){
  int n;
  data a[10];
  scanf("%d",&n);
  solve(n,a);
  while(scanf("%d",&n),n){
    puts("");
    solve(n,a);
  }
  return 0;
}