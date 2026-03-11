#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#include <string.h>





#define MAX_INT 2147483647
#define MIN_INT -2147483648





/*以下便利なマクロを定義する。*/

#define rep(i, min, max) for(i=min; i<=max; i++)



#define if_forall(i, min, max, prop)\
\
	for(i=min; i<=max; i++)\
	{\
		if(!(prop))\
		{\
			break;\
		}\
	}\
\
	if(i==max+1)\



#define if_exists(i, min, max, prop)\
\
	rep(i, min, max)\
	{\
		if(prop)\
		{\
			break;\
		}\
	}\
\
	if(i<max+1)\



#define sum(i, min, max, sequence, answer)\
{\
	answer=0;\
	rep(i, min, max)\
	{\
		answer=answer+sequence;\
	}\
}\



#define prod(i, min, max, sequence, answer)\
{\
	answer=1;\
	rep(i, min, max)\
	{\
		answer=answer*sequence;\
	}\
}\











/*以下便利な関数を定義する。*/

void fill_int_array(int array[], int min, int max, int element)
{
	int i=min;

	for(i=min; i<=max; i++)
	{
		array[i]=element;
	}
}



void fill_int_2dim_array(int size_1, int size_2, int array[size_1][size_2], int i_min, int i_max, int j_min, int j_max, int element)
{
	int i=i_min;

	for(i=i_min; i<=i_max; i++)
	{
		fill_int_array(array[i], j_min, j_max, element);
	}
}



void fill_char_array(char array[], int min, int max, char element)
{
	int i=min;

	for(i=min; i<=max; i++)
	{
		array[i]=element;
	}
}


void fill_char_2dim_array(int size_1, int size_2, char array[size_1][size_2], int min, int max, char element[])
{
	int i;

	for(i=min; i<=max; i++)
	{
		fill_char_array(array[i], 0, size_2-2, '\0');
		strcpy(array[i], element);
	}
}



void  swap_int(int* x, int* y)
{
    int  tmp;

    tmp = *x;
    *x = *y;
    *y = tmp;
}



void  swap_int_array(int x[], int y[], int min, int max)
{
	int i;

    rep(i, min, max)
	{
		swap_int(&x[i], &y[i]);
	}
}



void  swap_char(char* x, char* y)
{
    char  tmp;

    tmp = *x;
    *x = *y;
    *y = tmp;
}



void  swap_char_array(char x[], char y[], int min, int max)
{
	int i;

    rep(i, min, max)
	{
		swap_char(&x[i], &y[i]);
	}
}



void reverse_char(char array[], int min, int max)
{
	int i;

	rep(i, min, (min+max)/2)
	{
		swap_char(&array[i], &array[min+max-i]);
	}
}



int max_int(int var1, int var2)
{
	int answer=var1;

	if(answer<var2)
	{
		answer=var2;
	}

	return answer;
}



int min_int(int var1, int var2)
{
	int answer=var1;

	if(answer>var2)
	{
		answer=var2;
	}

	return answer;
}



double max_double(double var1, double var2)
{
	double answer=var1;

	if(answer<var2)
	{
		answer=var2;
	}

	return answer;
}



double min_double(double var1, double var2)
{
	double answer=var1;

	if(answer>var2)
	{
		answer=var2;
	}

	return answer;
}



long int pow_int(int base, unsigned int exponent)
{
	long int answer=1;

	answer=(long int)pow((double)base, (double)exponent);

	return answer;
}



long int pow_int_mod(int base, unsigned int exponent, int mod)
{
	long int answer=1;
	while (exponent>0)
	{
		if (exponent&1)
		{
			answer=(answer*(base%mod))%mod;
			if(answer==0)
			{
				break;
			}
		}
		base=(base*base)%mod;
		exponent=exponent>>1;
	}
	return answer;
}



unsigned long int factorial(unsigned int num)
{
	unsigned int i=0;
	unsigned long int answer=1;
	rep(i, 2, num)
	{
		answer=answer*i;
	}
	return answer;
}



unsigned long int factorial_mod(unsigned int num, int mod)
{
	unsigned int i=0;
	unsigned long int answer=1;
	rep(i, 2, num)
	{
		answer=(answer*(i%mod))%mod;
		if(answer==0)
		{
			break;
		}
	}
	return answer;
}



unsigned long int combination(unsigned int n, unsigned int k)
{
	unsigned int i=0;
	unsigned long int numerator=1;
	unsigned long int denominator=1;

	k=min_int(k, n-k);

	rep(i, 2, k)
	{
		numerator=numerator*(n+1-i);
		denominator=denominator*i;
	}
	return numerator/denominator;
}



double combination_general(double alpha, unsigned int k)
{
	unsigned int i;
	double numerator=1;
	unsigned long int denominator=1;

	rep(i, 1, k)
	{
		numerator=numerator*(alpha+1-i);
		denominator=denominator*i;
	}
	return numerator/denominator;
}



int gcd(int a, int b)
{
	if(a<b)
	{
		swap_int(&a, &b);
	}

	int r=a%b;

	while(r!=0)
	{
		a=b;
		b=r;
		r=a%b;
	}

	return b;
}



int lcm(int a, int b)
{
	return (a/gcd(a, b))*b;
}



int order(long int num)
{
	int answer=-1;
	while(num!=0)
	{
		num=num/10;
		answer=answer+1;
	}
	return answer;
}



long int convert_adic_int(char num[], unsigned int p_adic, unsigned int q_adic)
{
	int i=0;
	long int answer=0;
	bool sgn=0;
	long int tmp=strtol(num, NULL, p_adic);

	if(tmp<0)
	{
		sgn=1;
		tmp=-tmp;
	}

	while(tmp>0)
	{
		answer=answer+(tmp%q_adic)*pow_int(10, i);
		tmp=tmp/q_adic;
		i=i+1;
	}

	if(sgn==0)
	{
		return answer;
	}
	else
	{
		return -answer;
	}
}



void convert_adic_char(char num[], unsigned int p_adic, unsigned int q_adic)
{
	int i=0;
	int mod[32]={0};
	bool sgn=0;
	int digit=0;
	long int tmp=strtol(num, NULL, p_adic);

	if(tmp<0)
	{
		sgn=1;
		tmp=-tmp;
	}

	while(tmp>0)
	{
		mod[i]=tmp%q_adic;

		digit=i;
		tmp=tmp/q_adic;
		i=i+1;
	}

	fill_char_array(num, 0, min_int(0, strlen(num)-1), '\0');

	if(sgn==1)
	{
		num[0]='-';
	}

	rep(i, 0, digit)
	{
		if(mod[i]<10)
		{
			num[sgn+digit-i]='0'+mod[i];
		}
		else if(mod[i]<q_adic)
		{
			num[sgn+digit-i]='a'+(mod[i]-10);
		}
	}
}



int digit(long int num)
{
	int answer=0;
	while(num!=0)
	{
		num=num/10;
		answer=answer+1;
	}
	return answer;
}



int sum_digit(int num)
{
	int answer=0;
	num=abs(num);

	while(num>0)
	{
		answer=answer+num%10;
		num=num/10;
	}
	return answer;
}



int count_mod(int min, int max, int num)
{
	if(min>0)
	{
		return max/num-(min-1)/num;
	}
	else if(min==0)
	{
		return max/num+1;
	}
	else
	{
		return -1;
	}
}



void shift_char(char array[], int min, int max, int num)
{
	int i;
	int l=strlen(array);

	if(num>0)
	{
		max=min_int(max, l-num-1);

		for(i=max; i>=min; i=i-1)
		{
			array[i+num]=array[i];
		}

		for(i=min; i<=min+num-1; i=i+1)
		{
			array[i]='\0';
		}
	}
	else if(num<0)
	{
		min=max_int(min, -num);

		for(i=min; i<=max; i=i+1)
		{
			array[i+num]=array[i];
		}

		for(i=max+num+1; i<=max; i=i+1)
		{
			array[i]='\0';
		}
	}
}



void shift_int_array(int size, int array[size], int min, int max, int num)
{
	int i;

	if(num>0)
	{
		max=min_int(max, size-num-1);

		for(i=max; i>=min; i=i-1)
		{
			array[i+num]=array[i];
		}

		for(i=min; i<=min+num-1; i=i+1)
		{
			array[i]=0;
		}
	}
	else if(num<0)
	{
		min=max_int(min, -num);

		for(i=min; i<=max; i=i+1)
		{
			array[i+num]=array[i];
		}

		for(i=max+num+1; i<=max; i=i+1)
		{
			array[i]=0;
		}
	}
}



void convert_char(char array[], int min, int max, char pre, char post)
{
	int i=min;

	rep(i, min, max)
	{
		if(array[i]==pre)
		{
			array[i]=post;
		}
	}
}



int asc_int(const void *num1, const void *num2)
{
	return *(int*)num1-*(int*)num2;
}



int des_int(const void *num1, const void *num2)
{
	return *(int*)num2-*(int*)num1;
}



int asc_char_dic(const void *c1, const void *c2)
{
	return strcmp((char*)c1, (char*)c2);
}



int des_char_dic(const void *c1, const void *c2)
{
	return strcmp((char*)c2, (char*)c1);
}



void sort_asc_int(int array[], int min, int max)
{
	qsort(array+min, max-min+1, sizeof(int), asc_int);
}



void sort_des_int(int array[], int min, int max)
{
	qsort(array+min, max-min+1, sizeof(int), des_int);
}



void sort_asc_char_dic(char array[], int min, int max)
{
	qsort(array+min, max-min+1, sizeof(char), asc_char_dic);
}



void sort_des_char_dic(char array[], int min, int max)
{
	qsort(array+min, max-min+1, sizeof(char), des_char_dic);
}



void sort_asc_char_array_dic(int size_1, int size_2, char array[size_1][size_2], int min, int max)
{
	qsort(array[min], max-min+1, sizeof(char)*size_2, asc_char_dic);
}



void sort_des_char_array_dic(int size_1, int size_2, char array[size_1][size_2], int min, int max)
{
	qsort(array[min], max-min+1, sizeof(char)*size_2, des_char_dic);
}



int max_int_array(int array[], int min, int max)
{
	int i=min;
	int answer=array[min];

	for(i=min+1; i<=max; i=i+1)
	{
		if(answer<array[i])
		{
			answer=array[i];
		}
	}

	return answer;
}



int min_int_array(int array[], int min, int max)
{
	int i;
	int answer;

	answer=array[min];

	for(i=min+1; i<=max; i=i+1)
	{
		answer=min_int(answer, array[i]);
	}

	return answer;
}



char *max_char_2dim_array_dic(int size_1, int size_2, char array[size_1][size_2], int min, int max)
{
	int i;
	int answer=min;

	for(i=min+1; i<=max; i=i+1)
	{
		if(strcmp(array[answer], array[i])<0)
		{
			answer=i;
		}
	}

	return array[answer];
}



char *min_char_2dim_array_dic(int size_1, int size_2, char array[size_1][size_2], int min, int max)
{
	int i;
	int answer=min;

	for(i=min+1; i<=max; i=i+1)
	{
		if(strcmp(array[answer], array[i])>0)
		{
			answer=i;
		}
	}

	return array[answer];
}



int sum_array(int array[], int min, int max)
{
	int i=min;
	int answer=0;

	for(i=min; i<=max; i=i+1)
	{
		answer=answer+array[i];
	}

	return answer;
}



bool detect_int(int array[], int min, int max, int element)
{
	int i;
	bool answer=0;

	if_exists(i, min, max, array[i]==element)
	{
		answer=1;
	}

	return answer;
}



bool detect_char(char array[], int min, int max, char element)
{
	int i;
	bool answer=0;

	if_exists(i, min, max, array[i]==element)
	{
		answer=1;
	}
	return answer;
}



bool detect_char_array(char array[], int min, int max, char element[], int start, int goal)
{
	int i, j;
	bool answer=0;

	if(goal-start<=max-min)
	{
		for(i=min; i<=max-(goal-start); i++)
		{
			if_forall(j, start, goal, array[i+j-start]==element[j])
			{
				answer=1;
				break;
			}
		}
	}

	return answer;
}



int countd_int(int array[], int min, int max)
{
	int i;
	int answer=1;

	int copy[max-min+1];
	for(i=min; i<=max; i++)
	{
		copy[i-min]=array[i];
	}

	sort_asc_int(copy, min, max);

	for(i=min+1; i<=max; i++)
	{
		if(copy[i]!=copy[i-1])
		{
			answer++;
		}
	}
	return answer;
}



int countd_char(char array[], int min, int max)
{
	int i;
	int answer=1;

	char copy[max-min+1];
	for(i=min; i<=max; i++)
	{
		copy[i-min]=array[i];
	}

	sort_asc_char_dic(copy, min, max);

	for(i=min+1; i<=max; i++)
	{
		if(copy[i]!=copy[i-1])
		{
			answer++;
		}
	}
	return answer;
}


int counte_int(int array[], int min, int max, int element)
{
	int i;
	int answer=0;

	for(i=min; i<=max; i++)
	{
		if(array[i]==element)
		{
			answer++;
		}
	}
	return answer;
}



int counte_char(char array[], int min, int max, char element)
{
	int i;
	int answer=0;

	for(i=min; i<=max; i++)
	{
		if(array[i]==element)
		{
			answer++;
		}
	}
	return answer;
}


int counte_char_array(int size_1, int size_2, char array[size_1][size_2], int min, int max, char element[])
{
	int i;
	int answer=0;

	for(i=min; i<=max; i++)
	{
		if(strcmp(array[i], element)==0)
		{
			answer++;
		}
	}
	return answer;
}



void scanf_int_array(int array[], int min, int max)
{
	int i;
	for(i=min; i<=max; i=i+1)
	{
		scanf("%d", &array[i]);
	}
}



void scanf_int_array_2(int array1[], int array2[], int min, int max)
{
	int i;
	for(i=min; i<=max; i=i+1)
	{
		scanf("%d %d", &array1[i], &array2[i]);
	}
}



void scanf_int_array_3(int array1[], int array2[], int array3[], int min, int max)
{
	int i;
	for(i=min; i<=max; i=i+1)
	{
		scanf("%d %d %d", &array1[i], &array2[i], &array3[i]);
	}
}



void printf_int_array(int array[], int min, int max)
{
	int i;
	for(i=min; i<=max; i++)
	{
		printf("%d\n", array[i]);
	}
}



void printf_int_2dim_array(int size_1, int size_2, int array[size_1][size_2], int i_min, int i_max, int j_min, int j_max)
{
	int i, j;
	for(i=i_min; i<=i_max; i++)
	{
		for(j=j_min; j<=j_max; j++)
		{
			printf("%d ", array[i][j]);
		}
		printf("\n");
	}
}



void printf_char_array(char array[], int min, int max)
{
	int i;
	for(i=min; i<=max; i++)
	{
		printf("%c", array[i]);
	}
}



void printf_char_2dim_array(int size_1, int size_2, char array[size_1][size_2], int min, int max)
{
	int i;
	for(i=min; i<=max; i++)
	{
		printf("%s\n", array[i]);
	}
}



bool graph_existence_of_path(int size, int adjacency_matrix[size][size], int start, int goal)
{
	int i,j;
	int copy_matrix[size][size];
	for(i=0; i<size; i++)
	{
		for(j=0; j<size; j++)
		{
			copy_matrix[i][j]=adjacency_matrix[i][j];
		}
	}

	if(adjacency_matrix[start][goal]>0)
	{
		return 1;
	}
	else
	{
		for(i=0; i<size; i++)
		{
			if(adjacency_matrix[start][i]>0)
			{
				copy_matrix[start][i]=0;
				copy_matrix[i][start]=0;

				if(graph_existence_of_path(size, copy_matrix, i, goal)>0)
				{
					return 1;
				}

				copy_matrix[start][i]=1;
				copy_matrix[i][start]=1;
			}
		}
	}
	return 0;
}












int main(void)
{
	int i, k;
	int flag=1;

	int answer=0;

	int N, M;
	scanf("%d %d", &N, &M);

	int a[M], b[M];
	scanf_int_array_2(a,b,0,M-1);

	int adjacency_matrix[N][N];
	fill_int_2dim_array(N,N,adjacency_matrix,0,N-1,0,N-1,0);
	rep(i, 0, M-1)
	{
		adjacency_matrix[a[i]-1][b[i]-1]++;
		adjacency_matrix[b[i]-1][a[i]-1]++;
	}

	rep(k, 0, M-1)
	{
		adjacency_matrix[a[k]-1][b[k]-1]=0;
		adjacency_matrix[b[k]-1][a[k]-1]=0;

		rep(i,0,N-1)
		{
			if(graph_existence_of_path(N,adjacency_matrix,a[i]-1,b[i]-1)==0)
			{
				flag=0;
				break;
			}
		}

		if(flag==0)
		{
			answer++;
		}

		adjacency_matrix[a[k]-1][b[k]-1]=1;
		adjacency_matrix[b[k]-1][a[k]-1]=1;
		flag=1;
	}

	printf("%d\n", answer);

	return 0;
}
