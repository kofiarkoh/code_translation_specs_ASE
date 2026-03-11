#include<stdio.h>
int main(){
  int n,i,j,k=0;
  int A[100001];
  scanf("%d",&n);
  for(i=0;i<100001;i++)A[i]=0;
  for(i=0;i<n;i++){scanf("%d",&j);if(!A[j])k++,A[j]++;}
  printf("%d",k-!(k%2));
}
