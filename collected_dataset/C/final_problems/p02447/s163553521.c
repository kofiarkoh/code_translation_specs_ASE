#include <stdio.h>
#include <stdlib.h>


#define	ll_int long long int


typedef struct{
	ll_int x;
	ll_int y;
}DATA_T;


DATA_T *WORK;


void Merge_sort(DATA_T *box, int l, int r, int kind)
{
	int mid;
	ll_int num1, num2;
	int i, j, k;


	if(r <= l)
		return;

	mid = (l + r) / 2;


	Merge_sort(box, l, mid, kind);

	Merge_sort(box, mid + 1, r, kind);


	for(i = l; i <= mid; i++)
		WORK[i] = box[i];

	for(i = mid + 1, j = r; i <= r; i++, j--)
		WORK[i] = box[j];


	j = l;
	k = r;
	for(i = l; i <= r; i++){
		if(kind){
			num1 = WORK[j].y;
			num2 = WORK[k].y;
		}else{
			num1 = WORK[j].x;
			num2 = WORK[k].x;
		}

		if(num1 <= num2 && j <= mid)
			box[i] = WORK[j++];
		else
			box[i] = WORK[k--];
	}
}


void Print_box(DATA_T *box, int n)
{
	int i;

	for(i = 0; i < n; i++)
		printf("%lld %lld\n", box[i].x, box[i].y);
}


int main()
{
	int n;
	DATA_T *c_box;	//Coordinate
	int i;

	scanf("%d", &n);

	c_box = (DATA_T *)malloc(sizeof(DATA_T) * n);
	WORK = (DATA_T *)malloc(sizeof(DATA_T) * n);

	for(i = 0; i < n; i++)
		scanf("%lld %lld", &c_box[i].x, &c_box[i].y);

	Merge_sort(c_box, 0, n - 1, 1);
	Merge_sort(c_box, 0, n - 1, 0);

	Print_box(c_box, n);

	return 0;
}
