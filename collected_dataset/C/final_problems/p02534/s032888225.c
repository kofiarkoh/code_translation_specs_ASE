#include<stdio.h>
 
int main()
{
  int k;
  scanf("%d",&k);
  if(k==1){
    printf("ACL");
  }
  else if(k==2){
    printf("ACLACL");
  }
   else if(k==3){
    printf("ACLACLACL");
  }
   else if(k==4){
    printf("ACLACLACLACL");
  }
   else if(k==5){
    printf("ACLACLACLACLACL");
  }
}