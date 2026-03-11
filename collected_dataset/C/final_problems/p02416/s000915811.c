#include<stdio.h>
  
int main(void){
  
char num[1001];
  
int i=0;
  
int sum=1;
  
while(sum!=0){
  
sum=0;
  
i=0;
  
scanf("%c",&num[i]);
  
while(num[i]!='\n'){
  
i++;
  
scanf("%c",&num[i]);
  
}
  
num[i]='\0';
  
i=0;
  
while(num[i]!='\0'){
  
sum+=num[i]-'0';
  
i++;
  
}
  
if(sum!=0){
  
printf("%d\n",sum);
  
}
  
}
  
return 0;
  
}