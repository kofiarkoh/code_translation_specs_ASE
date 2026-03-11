#include<stdio.h>
#include<stdlib.h>
int cmp(const void *a, const void *b)
{
  return *(int*)a - *(int*)b;
}

int main()
{
  int n,l,r,i,j,ans;
  scanf("%d",&n);
  int a[n],b[n],m[n*n];
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
  }
  i = 0;
  for(l=0;l<n;l++){
    for(r=l;r<n;r++){
      for(j=0;j<=r-l;j++){
        b[j] = a[l+j];
      }
      qsort(b,j,sizeof(int),cmp);
      m[i] = b[((j-1)+(j-1)%2)/2];
      i++;
    }
  }
  qsort(m,i,sizeof(int),cmp);
  ans = m[((i-1)+(i-1)%2)/2];
  printf("%d",ans);
  return 0;
}
