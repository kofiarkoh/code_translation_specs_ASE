 #include<stdio.h>
 int main(void)
 {
     int a,b;
     int i,j,S,S1=1000,S2=1000;

     scanf("%d%d",&a,&b);

       S=0;
     for(i=1;i<=999;i++){
       S+=i;
       if(S>=a){
         S1=S-a;
         break;
       }
     }
       S=0;
     for(i=1;i<=999;i++){
       S+=i;
       if(S>=b){
         S2=S-b;
         break;
       }
     }

     if(S1==S2) printf("%d\n",S1);
     else if(S1==0) printf("%d\n",a);
     else if(S2==0) printf("%d\n",b);

 }

