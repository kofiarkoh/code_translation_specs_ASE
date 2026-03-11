#include <stdio.h>

#define MULTIPLICATION_THREE 3
#define MULTIPLICATION_TEN 10
//printf("helloWorld\n");


int main(void)
{
	//�ϐ���`
	int iInput;								//����n
	int iCount;							//�J�E���g�A�b�v�ϐ�
	int iDataNoTouch;
	int iMultiplicationResult;		//SEED_MULTIPLICATION�̔{��
	int iDataMultipleTen;

	//�ϐ��̏�����
	iInput = 0;
	iMultiplicationResult = 0;
	iDataNoTouch = 0;
	iDataMultipleTen = 0;
	iCount = 1;

	//����n�̓���
	scanf("%d", &iInput);

	//����n�ɒB����܂Ń��[�v
	for (iCount = 1; iCount <= iInput; iCount++)
	{
		iDataMultipleTen = iCount;
		//iCount �� MULTIPLICATION_THREE�Ŋ��������ʁA�]�肪�o�Ȃ��Ƃ�iCount���o�͂���
		if (iDataMultipleTen % MULTIPLICATION_THREE == 0)
		{
			printf(" %d", iCount);
		}
		//iCount��MULTIPLICATION_TEN�Ŋ��������ʁA�]�肪3�ɂȂ�Ƃ�iCount���o�͂���
		else 
		{
			do 	
			{ 
				if (iDataMultipleTen % MULTIPLICATION_TEN == 3)
				{
					printf(" %d", iCount);
					iDataMultipleTen = 0;
				}
				//����ȊO�͂Ȃɂ����Ȃ�
				else
				{
					iDataMultipleTen /= MULTIPLICATION_TEN;
				}
			} while (iDataMultipleTen != 0);
		}
	}
	printf("\n");
	return 0;
}
