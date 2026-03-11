#include<stdio.h>
int a[501][501]={0},n,m,p,t,i,j,z,d,c,si,sj,qq,s,h,mm;
int main()
{
    scanf("%d %d %d %d",&n,&m,&t,&p);
    while(!((n==0)&&(m==0)&&(p==0)&&(t==0)))
    {
       si=0;
       sj=0;
       /*8
       for(i=1;i<=100;i=i+1)
         for(j=1;j<=100;j=j+1)
           a[i][j]=0;
           */
       //printf();
       for(i=1;i<=m;i=i+1)
         for(j=1;j<=n;j=j+1)
         {
              a[i][j]=1;
         }

       for(z=1;z<=t;z=z+1)
       {
            scanf("%d %d",&d,&c);
            if(d==1)
            {
                for(i=si;i<=m;i=i+1)
                   for(j=sj+c;j>=sj+1;j=j-1)
                    {
                        qq=sj+c-j;
                        if(sj+c+1+qq>n)
                        {
                            n=sj+c+1+qq;
                            for(mm=1;mm<=m;mm=mm+1)
                                a[mm][sj+c+1+qq]=0;
                        }
                        a[i][sj+c+1+qq]=a[i][sj+c+1+qq]+a[i][j];
                    }
               sj=sj+c;
            }
            if(d==2)
            {
                for(j=sj;j<=n;j=j+1)
                   for(i=si+c;i>=si+1;i=i-1)
                    {
                        qq=si+c-i;
                        if(si+c+1+qq>m)
                        {
                            m=si+c+1+qq;
                            for(mm=1;mm<=n;mm=mm+1)
                                a[si+c+1+qq][mm]=0;
                        }
                        a[si+c+1+qq][j]=a[si+c+1+qq][j]+a[i][j];
                    }
                si=si+c;
            }
            /*
            printf("----------\n");
            for(i=1;i<=2*m;i=i+1)
            {

               for(j=1;j<=2*n;j=j+1)
                 printf("%d ",a[i][j]);
               printf("\n");
            }
            printf("----------\n");
            printf("si=%d sj=%d\n",si,sj);
            */

       }
       /*
       printf("----------\n");
       for(i=1;i<=2*m;i=i+1)
       {

           for(j=1;j<=2*n;j=j+1)
            printf("%d ",a[i][j]);
           printf("\n");
       }
       printf("----------\n");
       printf("si=%d sj=%d\n",si,sj);
       */
       for(z=1;z<=p;z=z+1)
       {
           scanf("%d %d",&d,&c);
           d=d+1;
           c=c+1;
           printf("%d\n",a[c+si][d+sj]);
       }

       scanf("%d %d %d %d",&n,&m,&t,&p);
    }
    return 0;
}
