#include<stdio.h>
#include<string.h>
int main()
{
 int b,j,k,len,c;

 char a[200],sub[200];
 
 for(;;){
         scanf("%s",a);
         if(strcmp(a,"-")==0)break;
                                len=strlen(a);
                                scanf("%d",&b);
                                for(j=0;j<b;j++){
                                                 scanf("%d",&c);
                                                 for(k=0;k<c;k++)sub[k]=a[k];
                                                 for(k=c;k<len;k++)a[k-c]=a[k];
                                                 for(k=len-c;k<len;k++)a[k]=sub[k-(len-c)];
                                                 }
                                 printf("%s\n",a);
       }
 return 0;

}


 
