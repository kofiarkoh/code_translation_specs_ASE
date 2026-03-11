#include<stdio.h>
typedef long long ll;
int N,L;
ll s[100002];
int ans[100000];
int main(){
  int i;
  int a;
  scanf("%d %d",&N,&L);
    scanf("%d",&a);
    s[1] = a;
  for(i = 2; i <= N; i++){
    scanf("%d",&a);
    s[i] = s[i-1]+a;    
  }
 
  if(s[N] < L){printf("Impossible\n");return 0;}
  
  int l = 1, r = N,j = 0;
  while(r != l){
    if(s[r]-s[l-1] < L)break;
    
    if(s[r]-s[r-1] > s[l]-s[l-1]){
      ans[j++] = l++;
    }
    else {ans[j++] = r-1;
      r--;
    }
  }
  if(r == l){
    printf("Possible\n");
    for(i = 0; i < j; i++){
      printf("%d\n",ans[i]);
    }
  }
  else printf("Impossible\n");
  return 0;
}