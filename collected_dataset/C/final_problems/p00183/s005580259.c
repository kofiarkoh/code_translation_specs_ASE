

#include<stdio.h>
int main(){
  char c[9],i;
  int d[]={0,1,2,3,4,5,6,7,8,0,3,6,1,4,7,2,5,8,0,4,8,2,4,6};
  while(scanf(" %c",&c[0]),c[0]-'0'){
    for(i=1;i<9;i++){
      scanf(" %c",&c[i]);
    }
    for(i=0;i<8;i++){
      if(c[d[i*3+0]]==c[d[i*3+1]]&&c[d[i*3+1]]==c[d[i*3+2]]&&c[d[i*3+0]]-'+'){
	printf("%c\n",c[d[i*3]]);
	break;
      }
    }
      if(i==8)printf("NA\n");
  }
  return 0;
}