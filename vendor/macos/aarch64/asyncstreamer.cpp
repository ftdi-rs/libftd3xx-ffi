#include <thread>
#include <chrono>
#include "common.hpp"

static bool fifo_600mode;
static thread measure_thread;
static thread write_thread;
static thread read_thread;
//static const int BUFFER_LEN = 32*1024;
static const int BUFFER_LEN = 256*1024;
static const int QUEUE_LEN = 4;


static void write_test(FT_HANDLE handle)
{
	unique_ptr<unique_ptr<uint8_t[]>[]> buf(new unique_ptr<uint8_t[]>[QUEUE_LEN]);
	OVERLAPPED pOverlapped[QUEUE_LEN];
	uint8_t i = 0;
	uint8_t channel = 0;
	printf("write start\r\n");

		
	for (i  = 0; i < QUEUE_LEN; i++) 
	{
		ULONG count = 0;

		buf[i] = unique_ptr<uint8_t[]>(new uint8_t[BUFFER_LEN]);
		FT_InitializeOverlapped(handle, &pOverlapped[i]);

		FT_WritePipeAsync(handle, channel,
			buf[i].get(), BUFFER_LEN, &count, &pOverlapped[i]);
	}
		
	i = 0;
	while(!do_exit)
	{	
		ULONG count = 0;
		if(FT_OK != FT_GetOverlappedResult(handle, &pOverlapped[i], &count, TRUE))
		{
			do_exit = true;
			break;
		}
			
		tx_count += count;
		
		FT_WritePipeAsync(handle, channel,
			buf[i].get(), BUFFER_LEN, &count, &pOverlapped[i]);

		if(++i == QUEUE_LEN)
		{
			i = 0;
		}

	}	
	
	for (i  = 0; i < QUEUE_LEN; i++) 
	{
		FT_ReleaseOverlapped(handle, &pOverlapped[i]);
	}
	printf("write stopped\r\n");
}

static void read_test(FT_HANDLE handle)
{
	unique_ptr<unique_ptr<uint8_t[]>[]> buf(new unique_ptr<uint8_t[]>[QUEUE_LEN]);
	OVERLAPPED pOverlapped[QUEUE_LEN];
	uint8_t i = 0;
	uint8_t channel = 0;
	printf("Read start\r\n");

		
	for (i  = 0; i < QUEUE_LEN; i++) 
	{
		ULONG count = 0;

		buf[i] = unique_ptr<uint8_t[]>(new uint8_t[BUFFER_LEN]);
		FT_InitializeOverlapped(handle, &pOverlapped[i]);

		FT_ReadPipeAsync(handle, channel,
			buf[i].get(), BUFFER_LEN, &count, &pOverlapped[i]);
	}
		
	i = 0;
	while(!do_exit)
	{	
		ULONG count = 0;
		if(FT_OK != FT_GetOverlappedResult(handle, &pOverlapped[i], &count, TRUE))
		{
			do_exit = true;
			break;
		}
			
		rx_count += count;
		
		FT_ReadPipeAsync(handle, channel,
			buf[i].get(), BUFFER_LEN, &count, &pOverlapped[i]);

		if(++i == QUEUE_LEN)
		{
			i = 0;
		}

	}	
	
	for (i  = 0; i < QUEUE_LEN; i++) 
	{
		FT_ReleaseOverlapped(handle, &pOverlapped[i]);
	}
	printf("Read stopped\r\n");
}

static void show_help(const char *bin)
{
	printf("Usage: %s <out channel count> <in channel count> [mode]\r\n", bin);
	printf("  channel count: [0, 1] for 245 mode, [0-4] for 600 mode\r\n");
	printf("  mode: 0 = FT245 mode (default), 1 = FT600 mode\r\n");
}


static bool validate_arguments(int argc, char *argv[])
{
	if (argc != 3 && argc != 4)
		return false;

	if (argc == 4) {
		int val = atoi(argv[3]);
		if (val != 0 && val != 1)
			return false;
		fifo_600mode = (bool)val;
	}

	out_ch_cnt = atoi(argv[1]);
	in_ch_cnt = atoi(argv[2]);

	if ((in_ch_cnt == 0 && out_ch_cnt == 0) ||
			in_ch_cnt > 4 || out_ch_cnt > 4) {
		show_help(argv[0]);
		return false;
	}
	return true;
}

int main(int argc, char *argv[])
{

	get_version();

	if (!validate_arguments(argc, argv)) {
		show_help(argv[0]);
		return 1;
	}

	if (!get_device_lists(500))
		return 1;


	/* Must be called before FT_Create is called */
	turn_off_thread_safe();
	
	FT_HANDLE handle = NULL;

	FT_Create(0, FT_OPEN_BY_INDEX, &handle);
	if (!handle) {
		printf("Failed to create device\r\n");
		return -1;
	}
	FT_SetStreamPipe(handle, TRUE, TRUE,0, BUFFER_LEN);
	if (out_ch_cnt)
		write_thread = thread(write_test, handle);
	if (in_ch_cnt)
		read_thread = thread(read_test, handle);
	measure_thread = thread(show_throughput, handle);
	register_signals();

	if (write_thread.joinable())
		write_thread.join();
	if (read_thread.joinable())
		read_thread.join();
	if (measure_thread.joinable())
		measure_thread.join();
	
	FT_Close(handle);
	return 0;
}
