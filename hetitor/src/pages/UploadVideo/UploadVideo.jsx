import { useTus } from 'use-tus'
import {useCallback} from "react";
function UploadVideo() {
    const { upload, setUpload, isSuccess, error, remove } = useTus();

    const handleSetUpload = useCallback((event) => {
            const file = event.target.files.item(0);
            if (!file) {
                return;
            }
            setUpload(file, {
                endpoint: 'https://9811-5-249-105-126.ngrok-free.app/videos',
                metadata: {
                    filename: file.name,
                    filetype: file.type,
                },
            });
        },
        [setUpload]
    );

    const handleStart = useCallback(() => {
        if (!upload) {
            return;
        }
        // Start to upload the file.
        upload.start();
    }, [upload]);

    function render() {
        return (
            <div>
                <input type="file" onChange={handleSetUpload} />
                <button type="button" onClick={handleStart}>
                    Upload
                </button>
            </div>
        );
    }

    return render();
}

export default UploadVideo;