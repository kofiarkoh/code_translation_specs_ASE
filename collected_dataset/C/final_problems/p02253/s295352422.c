#include <stdio.h>
#include <stdlib.h>

typedef struct timeAct{
    int Tfirst;
    int Tend;
}timeAct;

timeAct med3(timeAct x, timeAct y, timeAct z)
{
    if( x.Tend < y.Tend )
    {
    	if( y.Tend < z.Tend )
    	{
    		return y;
    	}
    	else if( z.Tend < x.Tend )
    	{
    		return x;
    	}
    	else
    	{
    		return z;
    	}
    }
    else
    {
    	if( z.Tend < y.Tend )
    	{
    		return y;
    	}
    	else if( x.Tend < z.Tend )
    	{
    		return x;
    	}
    	else
    	{
    		return z;
    	}
    }
}
 
void quickSort(timeAct *A,int p, int r)
{
    if(p < r)
    {
    	int i=p, j=r;
    	timeAct tmp, pibot = med3( A[i], A[i+(j-i)/2], A[j] );
    	while(1)
    	{
    		while( A[i].Tend < pibot.Tend ) i++;
    		while( pibot.Tend < A[j].Tend ) j--;
    		if( i >= j ) break;

    		tmp = A[i];
    		A[i] = A[j];
    		A[j] = tmp;
    		i++;
    		j--;
    	}
        quickSort(A, p, i - 1);
        quickSort(A, j + 1, r);
    }
}

int main()
{
    int i,n,cnt;
    int end=0;

    scanf("%d",&n);
    timeAct *ST = malloc(sizeof(timeAct)*n);

    /* データ読みとり */
    for( i=0 ; i<n ; i++ )
    {
        scanf("%d%d",&ST[i].Tfirst,&ST[i].Tend);
    }
//    for(i=0;i<n;i++) printf("%d %d\n",ST[i].Tfirst,ST[i].Tend);
//    printf("\n\n");

    quickSort(ST,0,n-1);

//    for(i=0;i<n;i++) printf("%d %d\n",ST[i].Tfirst,ST[i].Tend);
    end = ST[0].Tend;
    cnt = 1;
    for(i=1;i<n;i++)
    {
        if( ST[i].Tfirst > end )
        {
            cnt++;
            end = ST[i].Tend;
        }
    }
    printf("%d\n",cnt);
    return 0;
}
