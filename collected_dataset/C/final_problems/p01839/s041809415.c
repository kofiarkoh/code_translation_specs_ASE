#include<stdio.h>

int main(){
  int n, i, ans=0, cnt=0;
  char res[3];

  scanf("%d", &n);

  for(i=0; i<n; i++){
    scanf("%s", res);
    if(res[0]==65){
      cnt++;
    }else{
      cnt--;
    }

    if(cnt<0){
      ans=1;
    }
  }

  if(cnt) ans=1;

  if(ans){
    printf("NO\n");
  }else{
    printf("YES\n");
  }
  
  return 0;
}