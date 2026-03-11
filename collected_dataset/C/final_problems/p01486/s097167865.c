#include<stdio.h>
#include<string.h>

int main(void){
  char str[512],stack[512];
  int i,j,sp,l;
  //puts(emp);
  fgets(str,sizeof(str),stdin);
  l=strlen(str);
  //printf("%d\n",l);
  //puts(str);
  if(str[0]!='m') puts("Rabbit");
  else{
    sp=0;
    for(i=0;i<l-1;i++){
      stack[sp]=str[i];
      if(i>=2&&stack[sp-2]=='m'&&stack[sp-1]=='e'&&stack[sp]=='w'){
	for(j=sp;j>=sp-2;j--) stack[j]=' ';
	sp-=2;
	//printf("%d %d\n",sp,i);
	//puts(stack);
	if(i!=l-2&&sp==0){
	  //puts("out");
	  break;
	}
      }else sp++;
    }
    //printf("%d\n",i);
    if(sp==0&&i==l-1) puts("Cat");
    else puts("Rabbit");
  }   
  return 0;
}