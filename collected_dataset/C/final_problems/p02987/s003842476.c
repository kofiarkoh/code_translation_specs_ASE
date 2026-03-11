#include <stdio.h>

int main(){
  char a[5];
  int count[26]={0};
  int i,j,flag=0;
  scanf("%s",a);
  
  for(i=0;i<4;i++){
  	for(j=0;j<26;j++){
    	if(a[i]=='A'+j) count[j]++;
    }
  }
  
  for(j=0;j<26;j++){
    //printf("count[%d]=%d\n",j,count[j]);
    if(count[j]==0 || count[j]==2) flag =0;
    else{
      flag=1;
      break;
    }
  }
  
  if(flag == 1) printf("No\n");
  else printf("Yes\n");
  
  return 0;
  
}