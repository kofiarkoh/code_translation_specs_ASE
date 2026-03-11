#include <stdio.h>

int main(){
  int n,q;
  scanf("%d %d\n",&n,&q);
  char s[n];
  for(int i = 0; i < n; ++i) scanf("%c",&s[i]);
  char bin;
  scanf("%c",&bin);// \n
  int first[q],last[q],count[n];
  int flag;
  flag = 0;
  for(int i = 0; i < n; ++i) count[i] = 0;
  for(int i = 0; i < n; ++i){
    if(s[i] == 'C' && flag == 1){
      count[i-1] = 1;// count[i,i+1] = AC
      flag = 0;
    }
    else if(s[i] == 'A') flag = 1;
    else flag = 0;
  }
  //for(int i = 0; i < n; ++i) printf("%d",count[i]);
  int num;
  for(int j = 0; j < q; j++){
    scanf("%d %d\n",&first[j],&last[j]);
    num = 0;
    for(int k = first[j] - 1; k < last[j] - 1; ++k){
      if(count[k] == 1) num++;
     // printf("%d %d\n",k,count[k]);
    }
    printf("%d\n",num);
  }
  return 0;
}