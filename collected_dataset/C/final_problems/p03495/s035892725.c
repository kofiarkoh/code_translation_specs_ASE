#include<stdio.h>
#include<stdlib.h>

int compare_int(const void*a, const void*b)
{
  return *(int*)a-*(int*)b;
}

int main()
{
  int n,k;
  int a[200001];
  int i;
  int b[200001];
  int c=0;
  int ans=0;
  scanf("%d%d",&n,&k);
  for(i=0;i<n;i++){
    b[i]=0;
  }
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
    if(b[a[i]-1]==0) c++;
    b[a[i]-1]++;
  }
  qsort(b,n,sizeof(int),compare_int);
  i=0;
  while(c>k){
    if(b[i]!=0){
      c--;
      ans+=b[i];
    }
    i++;
  }
  printf("%d",ans);
  return 0;
}
