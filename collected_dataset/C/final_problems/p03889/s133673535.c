#include <stdio.h>

int main(){
  int i, j, k, l;
  char str1[100000], str2[100000];
  k=0;
  l=1;
  scanf("%s", str1);
  for(i=0;str1[i]!='\0';i++){
    str2[i]=str1[i];
    
  }
  str2[i]='\0';
  for(i=0;str2[i]!='\0';i++){
    if(str2[i]=='b'){
      str2[i]='d';
    }else if(str2[i]=='d'){
      str2[i]='b';
    }else if(str2[i]=='p'){
      str2[i]='q';
    }else if(str2[i]=='q'){
      str2[i]='p';
    }
     
  }
  for(j=i-1;j>=0;j--){
    if (str2[k]!=str1[j]){
      l =0;
    }
    k++;
  }
  if (l==1){
    printf("Yes\n");
  }else{
    printf("No\n");
  }
  
 
}
